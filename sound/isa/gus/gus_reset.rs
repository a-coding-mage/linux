// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Dependencies from the original C source:
// linux/delay.h, linux/interrupt.h, linux/time.h, asm/dma.h,
// sound/core.h, sound/gus.h

use core::ptr;

/*
 *  ok.. default interrupt handlers...
 */

unsafe extern "C" fn snd_gf1_default_interrupt_handler_midi_out(gus: *mut snd_gus_card) {
    unsafe {
        (*gus).gf1.uart_cmd &= !0x20;
        snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd);
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_midi_in(gus: *mut snd_gus_card) {
    unsafe {
        (*gus).gf1.uart_cmd &= !0x80;
        snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd);
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_timer1(gus: *mut snd_gus_card) {
    unsafe {
        (*gus).gf1.timer_enabled &= !4;
        snd_gf1_i_write8(
            gus,
            SNDRV_GF1_GB_SOUND_BLASTER_CONTROL,
            (*gus).gf1.timer_enabled,
        );
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_timer2(gus: *mut snd_gus_card) {
    unsafe {
        (*gus).gf1.timer_enabled &= !8;
        snd_gf1_i_write8(
            gus,
            SNDRV_GF1_GB_SOUND_BLASTER_CONTROL,
            (*gus).gf1.timer_enabled,
        );
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_wave_and_volume(
    gus: *mut snd_gus_card,
    _voice: *mut snd_gus_voice,
) {
    unsafe {
        snd_gf1_i_ctrl_stop(gus, 0x00);
        snd_gf1_i_ctrl_stop(gus, 0x0d);
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_dma_write(gus: *mut snd_gus_card) {
    unsafe {
        snd_gf1_i_write8(gus, 0x41, 0x00);
    }
}

unsafe extern "C" fn snd_gf1_default_interrupt_handler_dma_read(gus: *mut snd_gus_card) {
    unsafe {
        snd_gf1_i_write8(gus, 0x49, 0x00);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_set_default_handlers(gus: *mut snd_gus_card, what: libc::c_uint) {
    unsafe {
        if what & SNDRV_GF1_HANDLER_MIDI_OUT != 0 {
            (*gus).gf1.interrupt_handler_midi_out =
                Some(snd_gf1_default_interrupt_handler_midi_out);
        }
        if what & SNDRV_GF1_HANDLER_MIDI_IN != 0 {
            (*gus).gf1.interrupt_handler_midi_in = Some(snd_gf1_default_interrupt_handler_midi_in);
        }
        if what & SNDRV_GF1_HANDLER_TIMER1 != 0 {
            (*gus).gf1.interrupt_handler_timer1 = Some(snd_gf1_default_interrupt_handler_timer1);
        }
        if what & SNDRV_GF1_HANDLER_TIMER2 != 0 {
            (*gus).gf1.interrupt_handler_timer2 = Some(snd_gf1_default_interrupt_handler_timer2);
        }
        if what & SNDRV_GF1_HANDLER_VOICE != 0 {
            let voice: *mut snd_gus_voice = &mut (*gus).gf1.voices[(what & 0xffff) as usize];

            (*voice).handler_wave = Some(snd_gf1_default_interrupt_handler_wave_and_volume);
            (*voice).handler_volume = Some(snd_gf1_default_interrupt_handler_wave_and_volume);
            (*voice).handler_effect = None;
            (*voice).volume_change = None;
        }
        if what & SNDRV_GF1_HANDLER_DMA_WRITE != 0 {
            (*gus).gf1.interrupt_handler_dma_write =
                Some(snd_gf1_default_interrupt_handler_dma_write);
        }
        if what & SNDRV_GF1_HANDLER_DMA_READ != 0 {
            (*gus).gf1.interrupt_handler_dma_read = Some(snd_gf1_default_interrupt_handler_dma_read);
        }
    }
}

/*

 */

unsafe fn snd_gf1_clear_regs(gus: *mut snd_gus_card) {
    unsafe {
        // guard(spinlock_irqsave)(&gus->reg_lock);
        inb(GUSP(gus, IRQSTAT));
        snd_gf1_write8(gus, 0x41, 0); /* DRAM DMA Control Register */
        snd_gf1_write8(gus, 0x45, 0); /* Timer Control */
        snd_gf1_write8(gus, 0x49, 0); /* Sampling Control Register */
    }
}

unsafe fn snd_gf1_look_regs(gus: *mut snd_gus_card) {
    unsafe {
        // guard(spinlock_irqsave)(&gus->reg_lock);
        snd_gf1_look8(gus, 0x41); /* DRAM DMA Control Register */
        snd_gf1_look8(gus, 0x49); /* Sampling Control Register */
        inb(GUSP(gus, IRQSTAT));
        snd_gf1_read8(gus, 0x0f); /* IRQ Source Register */
    }
}

/*
 *  put selected GF1 voices to initial stage...
 */

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_smart_stop_voice(gus: *mut snd_gus_card, voice: libc::c_ushort) {
    unsafe {
        // guard(spinlock_irqsave)(&gus->reg_lock);
        snd_gf1_select_voice(gus, voice);
        /*
        dev_dbg(gus->card->dev,
            " -%i- smart stop voice - volume = 0x%x\n",
            voice, snd_gf1_i_read16(gus, SNDRV_GF1_VW_VOLUME));
        */
        snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_ADDRESS_CONTROL);
        snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_stop_voice(gus: *mut snd_gus_card, voice: libc::c_ushort) {
    unsafe {
        // guard(spinlock_irqsave)(&gus->reg_lock);
        snd_gf1_select_voice(gus, voice);
        /*
        dev_dbg(gus->card->dev,
            " -%i- stop voice - volume = 0x%x\n",
            voice, snd_gf1_i_read16(gus, SNDRV_GF1_VW_VOLUME));
        */
        snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_ADDRESS_CONTROL);
        snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL);
        if (*gus).gf1.enh_mode != 0 {
            snd_gf1_write8(gus, SNDRV_GF1_VB_ACCUMULATOR, 0);
        }
    }
}

unsafe fn snd_gf1_clear_voices(
    gus: *mut snd_gus_card,
    v_min: libc::c_ushort,
    v_max: libc::c_ushort,
) {
    unsafe {
        let daddr: libc::c_uint = (*gus).gf1.default_voice_address << 4;
        let mut i: libc::c_ushort = v_min;

        while i <= v_max {
            /*
            if (gus->gf1.syn_voices)
                gus->gf1.syn_voices[i].flags = ~VFLG_DYNAMIC;
            */
            // guard(spinlock_irqsave)(&gus->reg_lock);
            snd_gf1_select_voice(gus, i);
            snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_ADDRESS_CONTROL); /* Voice Control Register = voice stop */
            snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL); /* Volume Ramp Control Register = ramp off */
            if (*gus).gf1.enh_mode != 0 {
                snd_gf1_write8(
                    gus,
                    SNDRV_GF1_VB_MODE,
                    if (*gus).gf1.memory != 0 { 0x02 } else { 0x82 },
                ); /* Deactivate voice */
            }
            let w_16: libc::c_ushort =
                (snd_gf1_read8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL) & 0x04) as libc::c_ushort;
            snd_gf1_write16(gus, SNDRV_GF1_VW_FREQUENCY, 0x400);
            snd_gf1_write_addr(gus, SNDRV_GF1_VA_START, daddr, w_16);
            snd_gf1_write_addr(gus, SNDRV_GF1_VA_END, daddr, w_16);
            snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_START, 0);
            snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_END, 0);
            snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_RATE, 0);
            snd_gf1_write16(gus, SNDRV_GF1_VW_VOLUME, 0);
            snd_gf1_write_addr(gus, SNDRV_GF1_VA_CURRENT, daddr, w_16);
            snd_gf1_write8(gus, SNDRV_GF1_VB_PAN, 7);
            if (*gus).gf1.enh_mode != 0 {
                snd_gf1_write8(gus, SNDRV_GF1_VB_ACCUMULATOR, 0);
                snd_gf1_write16(gus, SNDRV_GF1_VW_EFFECT_VOLUME, 0);
                snd_gf1_write16(gus, SNDRV_GF1_VW_EFFECT_VOLUME_FINAL, 0);
            }
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_stop_voices(
    gus: *mut snd_gus_card,
    v_min: libc::c_ushort,
    v_max: libc::c_ushort,
) {
    unsafe {
        if in_interrupt() == 0 {
            /* this can't be done in interrupt */
            let mut i: libc::c_short = v_min as libc::c_short;
            let mut ramp_ok: libc::c_short = 0;
            while i <= v_max as libc::c_short {
                // guard(spinlock_irqsave)(&gus->reg_lock);
                snd_gf1_select_voice(gus, i as libc::c_ushort);
                let ramp_end: libc::c_ushort = (snd_gf1_read16(gus, 9) >> 8) as libc::c_ushort;
                if ramp_end > SNDRV_GF1_MIN_OFFSET {
                    ramp_ok += 1;
                    snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_RATE, 20); /* ramp rate */
                    snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_START, SNDRV_GF1_MIN_OFFSET); /* ramp start */
                    snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_END, ramp_end); /* ramp end */
                    snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, 0x40); /* ramp down */
                    if (*gus).gf1.enh_mode != 0 {
                        snd_gf1_delay(gus);
                        snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, 0x40);
                    }
                }
                i += 1;
            }
            msleep_interruptible(50);
        }
        snd_gf1_clear_voices(gus, v_min, v_max);
    }
}

unsafe fn snd_gf1_alloc_voice_use(
    gus: *mut snd_gus_card,
    pvoice: *mut snd_gus_voice,
    type_: libc::c_int,
    client: libc::c_int,
    port: libc::c_int,
) {
    unsafe {
        (*pvoice).use_ = 1;
        match type_ {
            SNDRV_GF1_VOICE_TYPE_PCM => {
                (*gus).gf1.pcm_alloc_voices += 1;
                (*pvoice).pcm = 1;
            }
            SNDRV_GF1_VOICE_TYPE_SYNTH => {
                (*pvoice).synth = 1;
                (*pvoice).client = client;
                (*pvoice).port = port;
            }
            SNDRV_GF1_VOICE_TYPE_MIDI => {
                (*pvoice).midi = 1;
                (*pvoice).client = client;
                (*pvoice).port = port;
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_alloc_voice(
    gus: *mut snd_gus_card,
    type_: libc::c_int,
    client: libc::c_int,
    port: libc::c_int,
) -> *mut snd_gus_voice {
    unsafe {
        // guard(spinlock_irqsave)(&gus->voice_alloc);
        if type_ == SNDRV_GF1_VOICE_TYPE_PCM {
            if (*gus).gf1.pcm_alloc_voices >= (*gus).gf1.pcm_channels {
                return ptr::null_mut();
            }
        }
        for idx in 0..32 {
            let pvoice: *mut snd_gus_voice = &mut (*gus).gf1.voices[idx];
            if (*pvoice).use_ == 0 {
                snd_gf1_alloc_voice_use(gus, pvoice, type_, client, port);
                return pvoice;
            }
        }
        for idx in 0..32 {
            let pvoice: *mut snd_gus_voice = &mut (*gus).gf1.voices[idx];
            if (*pvoice).midi != 0 && (*pvoice).client == 0 {
                snd_gf1_clear_voices(gus, (*pvoice).number, (*pvoice).number);
                snd_gf1_alloc_voice_use(gus, pvoice, type_, client, port);
                return pvoice;
            }
        }
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_free_voice(gus: *mut snd_gus_card, voice: *mut snd_gus_voice) {
    unsafe {
        let private_free: Option<unsafe extern "C" fn(*mut snd_gus_voice)>;

        if voice.is_null() || (*voice).use_ == 0 {
            return;
        }
        snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_VOICE | (*voice).number as libc::c_uint);
        snd_gf1_clear_voices(gus, (*voice).number, (*voice).number);
        // scoped_guard(spinlock_irqsave, &gus->voice_alloc) {
        private_free = (*voice).private_free;
        (*voice).private_free = None;
        (*voice).private_data = ptr::null_mut();
        if (*voice).pcm != 0 {
            (*gus).gf1.pcm_alloc_voices -= 1;
        }
        (*voice).pcm = 0;
        (*voice).use_ = 0;
        (*voice).sample_ops = ptr::null_mut();
        // }
        if let Some(private_free_fn) = private_free {
            private_free_fn(voice);
        }
    }
}

unsafe fn snd_gf1_init_software_state(gus: *mut snd_gus_card) {
    unsafe {
        snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_ALL);
        for i in 0..32 {
            (*gus).gf1.voices[i].number = i as libc::c_ushort;
            snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_VOICE | i as libc::c_uint);
        }
    }
}

unsafe fn snd_gf1_hw_start(gus: *mut snd_gus_card, initial: bool) {
    unsafe {
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 0); /* reset GF1 */
        udelay(160);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1); /* disable IRQ & DAC */
        udelay(160);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_JOYSTICK_DAC_LEVEL, (*gus).joystick_dac);

        if initial {
            snd_gf1_init_software_state(gus);
            snd_gf1_uart_cmd(gus, 0x03);
        } else {
            // guard(spinlock_irqsave)(&gus->uart_cmd_lock);
            outb(0x03, GUSP(gus, MIDICTRL));
        }

        if (*gus).gf1.enh_mode != 0 {
            /* enhanced mode !!!! */
            snd_gf1_i_write8(
                gus,
                SNDRV_GF1_GB_GLOBAL_MODE,
                snd_gf1_i_look8(gus, SNDRV_GF1_GB_GLOBAL_MODE) | 0x01,
            );
            snd_gf1_i_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x01);
        }
        snd_gf1_clear_regs(gus);
        snd_gf1_select_active_voices(gus);
        snd_gf1_delay(gus);
        (*gus).gf1.default_voice_address = if (*gus).gf1.memory > 0 { 0 } else { 512 - 8 };
        (*gus).gf1.hw_lfo = 0;
        (*gus).gf1.sw_lfo = 0;
        /* initialize LFOs & clear LFOs memory */
        if (*gus).gf1.enh_mode != 0 && (*gus).gf1.memory != 0 {
            (*gus).gf1.hw_lfo = 1;
            (*gus).gf1.default_voice_address += 1024;
        } else {
            (*gus).gf1.sw_lfo = 1;
        }

        if (*gus).gf1.memory > 0 {
            for i in 0..4 {
                snd_gf1_poke(gus, (*gus).gf1.default_voice_address + i, 0);
            }
        }
        snd_gf1_clear_regs(gus);
        snd_gf1_clear_voices(gus, 0, 31);
        snd_gf1_look_regs(gus);
        udelay(160);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 7); /* Reset Register = IRQ enable, DAC enable */
        udelay(160);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 7); /* Reset Register = IRQ enable, DAC enable */
        if (*gus).gf1.enh_mode != 0 {
            /* enhanced mode !!!! */
            snd_gf1_i_write8(
                gus,
                SNDRV_GF1_GB_GLOBAL_MODE,
                snd_gf1_i_look8(gus, SNDRV_GF1_GB_GLOBAL_MODE) | 0x01,
            );
            snd_gf1_i_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x01);
        }
        while (snd_gf1_i_read8(gus, SNDRV_GF1_GB_VOICES_IRQ) & 0xc0) != 0xc0 {}

        // scoped_guard(spinlock_irqsave, &gus->reg_lock) {
        (*gus).gf1.active_voice = 0;
        outb((*gus).gf1.active_voice, GUSP(gus, GF1PAGE));
        outb((*gus).mix_cntrl_reg, GUSP(gus, MIXCNTRLREG));
        // }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_start(gus: *mut snd_gus_card) -> libc::c_int {
    unsafe {
        /*
         * Probe-time startup initializes both GF1 hardware and the
         * software state that suspend/resume keeps across PM cycles.
         */
        snd_gf1_hw_start(gus, true);
        snd_gf1_timers_init(gus);
        snd_gf1_look_regs(gus);
        snd_gf1_mem_init(gus);
        snd_gf1_mem_proc_init(gus);
        // CONFIG_SND_DEBUG:
        // snd_gus_irq_profile_init(gus);

        /*
        if (gus->pnp_flag) {
            if (gus->chip.playback_fifo_size > 0)
                snd_gf1_i_write16(gus, SNDRV_GF1_GW_FIFO_RECORD_BASE_ADDR, gus->chip.playback_fifo_block->ptr >> 8);
            if (gus->chip.record_fifo_size > 0)
                snd_gf1_i_write16(gus, SNDRV_GF1_GW_FIFO_PLAY_BASE_ADDR, gus->chip.record_fifo_block->ptr >> 8);
            snd_gf1_i_write16(gus, SNDRV_GF1_GW_FIFO_SIZE, gus->chip.interwave_fifo_reg);
        }
        */

        0
    }
}

/*
 *  call this function only by shutdown of driver
 */

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_stop(gus: *mut snd_gus_card) -> libc::c_int {
    unsafe {
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, 0); /* stop all timers */
        snd_gf1_stop_voices(gus, 0, 31); /* stop all voices */
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1); /* disable IRQ & DAC */
        snd_gf1_timers_done(gus);
        snd_gf1_mem_done(gus);

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_suspend(gus: *mut snd_gus_card) -> libc::c_int {
    unsafe {
        snd_gf1_dma_suspend(gus);
        snd_gf1_uart_suspend(gus);

        snd_gf1_i_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, 0);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL, 0);
        snd_gf1_i_look8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL);
        snd_gf1_stop_voices(gus, 0, 31);
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1);
        snd_dma_disable((*gus).gf1.dma2);

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_resume(gus: *mut snd_gus_card) -> libc::c_int {
    unsafe {
        snd_gf1_hw_start(gus, false);
        snd_gf1_timers_resume(gus);
        snd_gf1_uart_resume(gus);

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
