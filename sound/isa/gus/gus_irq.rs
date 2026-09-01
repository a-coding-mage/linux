// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routine for IRQ handling from GF1/InterWave chip
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies in the original C source:
 * #include <sound/core.h>
 * #include <sound/info.h>
 * #include <sound/gus.h>
 */

#[cfg(CONFIG_SND_DEBUG)]
macro_rules! STAT_ADD {
    ($x:expr) => {
        $x = $x.wrapping_add(1)
    };
}

#[cfg(not(CONFIG_SND_DEBUG))]
macro_rules! STAT_ADD {
    ($x:expr) => {
        while false {}
    };
}

extern "C" {
    fn inb(port: u16) -> u8;
    fn IRQ_RETVAL(x: ::core::ffi::c_int) -> irqreturn_t;
    fn snd_gf1_i_read8(gus: *mut snd_gus_card, reg: ::core::ffi::c_int) -> u8;
    fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: ::core::ffi::c_int) -> u8;
    fn snd_gf1_i_ctrl_stop(gus: *mut snd_gus_card, reg: ::core::ffi::c_int);

    static IRQ_NONE: irqreturn_t;
    static SNDRV_GF1_GB_VOICES_IRQ: ::core::ffi::c_int;
    static SNDRV_GF1_VB_ADDRESS_CONTROL: ::core::ffi::c_int;
    static SNDRV_GF1_VB_VOLUME_CONTROL: ::core::ffi::c_int;
    static SNDRV_GF1_GB_DRAM_DMA_CONTROL: ::core::ffi::c_int;
    static SNDRV_GF1_GB_REC_DMA_CONTROL: ::core::ffi::c_int;
}

pub unsafe extern "C" fn snd_gus_interrupt(
    irq: ::core::ffi::c_int,
    dev_id: *mut ::core::ffi::c_void,
) -> irqreturn_t {
    let gus: *mut snd_gus_card = dev_id as *mut snd_gus_card;
    let mut status: u8;
    let mut loop_: ::core::ffi::c_int = 100;
    let mut handled: ::core::ffi::c_int = 0;

    loop {
        status = inb((*gus).gf1.reg_irqstat);
        if status == 0 {
            return IRQ_RETVAL(handled);
        }
        handled = 1;
        if status & 0x02 != 0 {
            STAT_ADD!((*gus).gf1.interrupt_stat_midi_in);
            if let Some(interrupt_handler_midi_in) = (*gus).gf1.interrupt_handler_midi_in {
                interrupt_handler_midi_in(gus);
            }
        }
        if status & 0x01 != 0 {
            STAT_ADD!((*gus).gf1.interrupt_stat_midi_out);
            if let Some(interrupt_handler_midi_out) = (*gus).gf1.interrupt_handler_midi_out {
                interrupt_handler_midi_out(gus);
            }
        }
        if status & (0x20 | 0x40) != 0 {
            let mut already: ::core::ffi::c_uint;
            let mut _current_: ::core::ffi::c_uint;
            let mut voice_status: u8;
            let mut voice: u8;
            let mut pvoice: *mut snd_gus_voice;

            already = 0;
            while {
                voice_status = snd_gf1_i_read8(gus, SNDRV_GF1_GB_VOICES_IRQ);
                voice_status & 0xc0 != 0xc0
            } {
                voice = voice_status & 0x1f;
                _current_ = 1u32.wrapping_shl(voice as u32) as ::core::ffi::c_uint;
                if already & _current_ != 0 {
                    continue; /* multi request */
                }
                already |= _current_; /* mark request */
                /*
                 * Disabled in the original C source:
                 * dev_dbg(gus->card->dev,
                 *     "voice = %i, voice_status = 0x%x, voice_verify = %i\n",
                 *     voice, voice_status, inb(GUSP(gus, GF1PAGE)));
                 */
                pvoice = &mut (*gus).gf1.voices[voice as usize];
                if (*pvoice).use_ != 0 {
                    if voice_status & 0x80 == 0 {
                        /* voice position IRQ */
                        STAT_ADD!((*pvoice).interrupt_stat_wave);
                        if let Some(handler_wave) = (*pvoice).handler_wave {
                            handler_wave(gus, pvoice);
                        }
                    }
                    if voice_status & 0x40 == 0 {
                        /* volume ramp IRQ */
                        STAT_ADD!((*pvoice).interrupt_stat_volume);
                        if let Some(handler_volume) = (*pvoice).handler_volume {
                            handler_volume(gus, pvoice);
                        }
                    }
                } else {
                    STAT_ADD!((*gus).gf1.interrupt_stat_voice_lost);
                    snd_gf1_i_ctrl_stop(gus, SNDRV_GF1_VB_ADDRESS_CONTROL);
                    snd_gf1_i_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL);
                }
            }
        }
        if status & 0x04 != 0 {
            STAT_ADD!((*gus).gf1.interrupt_stat_timer1);
            if let Some(interrupt_handler_timer1) = (*gus).gf1.interrupt_handler_timer1 {
                interrupt_handler_timer1(gus);
            }
        }
        if status & 0x08 != 0 {
            STAT_ADD!((*gus).gf1.interrupt_stat_timer2);
            if let Some(interrupt_handler_timer2) = (*gus).gf1.interrupt_handler_timer2 {
                interrupt_handler_timer2(gus);
            }
        }
        if status & 0x80 != 0 {
            if snd_gf1_i_look8(gus, SNDRV_GF1_GB_DRAM_DMA_CONTROL) & 0x40 != 0 {
                STAT_ADD!((*gus).gf1.interrupt_stat_dma_write);
                if let Some(interrupt_handler_dma_write) = (*gus).gf1.interrupt_handler_dma_write {
                    interrupt_handler_dma_write(gus);
                }
            }
            if snd_gf1_i_look8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL) & 0x40 != 0 {
                STAT_ADD!((*gus).gf1.interrupt_stat_dma_read);
                if let Some(interrupt_handler_dma_read) = (*gus).gf1.interrupt_handler_dma_read {
                    interrupt_handler_dma_read(gus);
                }
            }
        }
        loop_ -= 1;
        if loop_ <= 0 {
            break;
        }
    }
    IRQ_NONE
}

#[cfg(CONFIG_SND_DEBUG)]
extern "C" {
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const ::core::ffi::c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const ::core::ffi::c_char,
        private_data: *mut snd_gus_card,
        read: Option<
            unsafe extern "C" fn(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer),
        >,
    );
}

#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" fn snd_gus_irq_info_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut gus: *mut snd_gus_card;
    let mut pvoice: *mut snd_gus_voice;
    let mut idx: ::core::ffi::c_int;

    gus = (*entry).private_data as *mut snd_gus_card;
    snd_iprintf(
        buffer,
        b"midi out = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_midi_out,
    );
    snd_iprintf(
        buffer,
        b"midi in = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_midi_in,
    );
    snd_iprintf(
        buffer,
        b"timer1 = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_timer1,
    );
    snd_iprintf(
        buffer,
        b"timer2 = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_timer2,
    );
    snd_iprintf(
        buffer,
        b"dma write = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_dma_write,
    );
    snd_iprintf(
        buffer,
        b"dma read = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_dma_read,
    );
    snd_iprintf(
        buffer,
        b"voice lost = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*gus).gf1.interrupt_stat_voice_lost,
    );
    idx = 0;
    while idx < 32 {
        pvoice = &mut (*gus).gf1.voices[idx as usize];
        snd_iprintf(
            buffer,
            b"voice %i: wave = %u, volume = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            idx,
            (*pvoice).interrupt_stat_wave,
            (*pvoice).interrupt_stat_volume,
        );
        idx += 1;
    }
}

#[cfg(CONFIG_SND_DEBUG)]
pub unsafe extern "C" fn snd_gus_irq_profile_init(gus: *mut snd_gus_card) {
    snd_card_ro_proc_new(
        (*gus).card,
        b"gusirq\0".as_ptr() as *const ::core::ffi::c_char,
        gus,
        Some(snd_gus_irq_info_read),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
