// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Creative Labs, Inc.
 *  Routines for IRQ control of EMU10K1 chips
 */

// C dependencies: <linux/time.h>, <sound/core.h>, <sound/emu10k1.h>

pub unsafe extern "C" fn snd_emu10k1_interrupt(
    irq: ::core::ffi::c_int,
    dev_id: *mut ::core::ffi::c_void,
) -> irqreturn_t {
    let emu: *mut snd_emu10k1 = dev_id as *mut snd_emu10k1;
    let mut status: ::core::ffi::c_uint;
    let mut orig_status: ::core::ffi::c_uint;
    let mut handled: ::core::ffi::c_int = 0;
    let mut timeout: ::core::ffi::c_int = 0;

    loop {
        status = inl((*emu).port + IPR);
        if status == 0 {
            break;
        }

        handled = 1;
        if (status & 0xffffffff) == 0xffffffff {
            dev_info(
                (*(*emu).card).dev,
                b"Suspected sound card removal\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            break;
        }
        timeout += 1;
        if timeout == 1000 {
            dev_info(
                (*(*emu).card).dev,
                b"emu10k1 irq routine failure\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            break;
        }
        orig_status = status;
        if (status & IPR_PCIERROR) != 0 {
            dev_err(
                (*(*emu).card).dev,
                b"interrupt: PCI error\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            snd_emu10k1_intr_disable(emu, INTE_PCIERRORENABLE);
            status &= !IPR_PCIERROR;
        }
        if (status & (IPR_VOLINCR | IPR_VOLDECR | IPR_MUTE)) != 0 {
            if let Some(hwvol_interrupt) = (*emu).hwvol_interrupt {
                hwvol_interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(
                    emu,
                    INTE_VOLINCRENABLE | INTE_VOLDECRENABLE | INTE_MUTEENABLE,
                );
            }
            status &= !(IPR_VOLINCR | IPR_VOLDECR | IPR_MUTE);
        }
        if (status & IPR_CHANNELLOOP) != 0 {
            let mut pvoice: *mut snd_emu10k1_voice;
            let mut voice: ::core::ffi::c_int;
            let voice_max: ::core::ffi::c_int =
                (status & IPR_CHANNELNUMBERMASK) as ::core::ffi::c_int;
            let mut val: u32;

            val = snd_emu10k1_ptr_read(emu, CLIPL, 0);
            pvoice = (*emu).voices;
            voice = 0;
            while voice <= voice_max {
                if voice == 0x20 {
                    val = snd_emu10k1_ptr_read(emu, CLIPH, 0);
                }
                if (val & 1) != 0 {
                    if (*pvoice).use_ != 0 && (*pvoice).interrupt.is_some() {
                        if let Some(interrupt) = (*pvoice).interrupt {
                            interrupt(emu, pvoice);
                        }
                        snd_emu10k1_voice_intr_ack(emu, voice);
                    } else {
                        snd_emu10k1_voice_intr_disable(emu, voice);
                    }
                }
                val >>= 1;
                pvoice = pvoice.add(1);
                voice += 1;
            }
            val = snd_emu10k1_ptr_read(emu, HLIPL, 0);
            pvoice = (*emu).voices;
            voice = 0;
            while voice <= voice_max {
                if voice == 0x20 {
                    val = snd_emu10k1_ptr_read(emu, HLIPH, 0);
                }
                if (val & 1) != 0 {
                    if (*pvoice).use_ != 0 && (*pvoice).interrupt.is_some() {
                        if let Some(interrupt) = (*pvoice).interrupt {
                            interrupt(emu, pvoice);
                        }
                        snd_emu10k1_voice_half_loop_intr_ack(emu, voice);
                    } else {
                        snd_emu10k1_voice_half_loop_intr_disable(emu, voice);
                    }
                }
                val >>= 1;
                pvoice = pvoice.add(1);
                voice += 1;
            }
            status &= !(IPR_CHANNELLOOP | IPR_CHANNELNUMBERMASK);
        }
        if (status & (IPR_ADCBUFFULL | IPR_ADCBUFHALFFULL)) != 0 {
            if let Some(capture_interrupt) = (*emu).capture_interrupt {
                capture_interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_ADCBUFENABLE);
            }
            status &= !(IPR_ADCBUFFULL | IPR_ADCBUFHALFFULL);
        }
        if (status & (IPR_MICBUFFULL | IPR_MICBUFHALFFULL)) != 0 {
            if let Some(capture_mic_interrupt) = (*emu).capture_mic_interrupt {
                capture_mic_interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_MICBUFENABLE);
            }
            status &= !(IPR_MICBUFFULL | IPR_MICBUFHALFFULL);
        }
        if (status & (IPR_EFXBUFFULL | IPR_EFXBUFHALFFULL)) != 0 {
            if let Some(capture_efx_interrupt) = (*emu).capture_efx_interrupt {
                capture_efx_interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_EFXBUFENABLE);
            }
            status &= !(IPR_EFXBUFFULL | IPR_EFXBUFHALFFULL);
        }
        if (status & (IPR_MIDITRANSBUFEMPTY | IPR_MIDIRECVBUFEMPTY)) != 0 {
            if let Some(interrupt) = (*emu).midi.interrupt {
                interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_MIDITXENABLE | INTE_MIDIRXENABLE);
            }
            status &= !(IPR_MIDITRANSBUFEMPTY | IPR_MIDIRECVBUFEMPTY);
        }
        if (status & (IPR_A_MIDITRANSBUFEMPTY2 | IPR_A_MIDIRECVBUFEMPTY2)) != 0 {
            if let Some(interrupt) = (*emu).midi2.interrupt {
                interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_A_MIDITXENABLE2 | INTE_A_MIDIRXENABLE2);
            }
            status &= !(IPR_A_MIDITRANSBUFEMPTY2 | IPR_A_MIDIRECVBUFEMPTY2);
        }
        if (status & IPR_INTERVALTIMER) != 0 {
            if !(*emu).timer.is_null() {
                snd_timer_interrupt((*emu).timer, (*(*emu).timer).sticks);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_INTERVALTIMERENB);
            }
            status &= !IPR_INTERVALTIMER;
        }
        if (status & (IPR_GPSPDIFSTATUSCHANGE | IPR_CDROMSTATUSCHANGE)) != 0 {
            if let Some(spdif_interrupt) = (*emu).spdif_interrupt {
                spdif_interrupt(emu, status);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_GPSPDIFENABLE | INTE_CDSPDIFENABLE);
            }
            status &= !(IPR_GPSPDIFSTATUSCHANGE | IPR_CDROMSTATUSCHANGE);
        }
        if (status & IPR_FXDSP) != 0 {
            if let Some(dsp_interrupt) = (*emu).dsp_interrupt {
                dsp_interrupt(emu);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_FXDSPENABLE);
            }
            status &= !IPR_FXDSP;
        }
        if (status & IPR_P16V) != 0 {
            if let Some(p16v_interrupt) = (*emu).p16v_interrupt {
                p16v_interrupt(emu);
            } else {
                outl(0, (*emu).port + INTE2);
            }
            status &= !IPR_P16V;
        }
        if (status & IPR_A_GPIO) != 0 {
            if let Some(gpio_interrupt) = (*emu).gpio_interrupt {
                gpio_interrupt(emu);
            } else {
                snd_emu10k1_intr_disable(emu, INTE_A_GPIOENABLE);
            }
            status &= !IPR_A_GPIO;
        }

        if status != 0 {
            dev_err(
                (*(*emu).card).dev,
                b"unhandled interrupt: 0x%08x\n\0".as_ptr() as *const ::core::ffi::c_char,
                status,
            );
        }
        outl(orig_status, (*emu).port + IPR); /* ack all */
    }

    IRQ_RETVAL(handled)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
