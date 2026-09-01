// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for GF1 DMA control
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use crate::*;

unsafe fn snd_gf1_dma_ack(gus: *mut snd_gus_card) {
    let _guard = spinlock_irqsave_guard(&mut (*gus).reg_lock);
    snd_gf1_write8(gus, SNDRV_GF1_GB_DRAM_DMA_CONTROL, 0x00);
    snd_gf1_look8(gus, SNDRV_GF1_GB_DRAM_DMA_CONTROL);
}

unsafe fn snd_gf1_dma_program(
    gus: *mut snd_gus_card,
    addr: ::core::ffi::c_uint,
    buf_addr: ::core::ffi::c_ulong,
    mut count: ::core::ffi::c_uint,
    cmd: ::core::ffi::c_uint,
) {
    let address: ::core::ffi::c_uint;
    let mut dma_cmd: ::core::ffi::c_uchar;
    let address_high: ::core::ffi::c_uint;

    dev_dbg(
        (*(*gus).card).dev,
        c"dma_transfer: addr=0x%x, buf=0x%lx, count=0x%x\n".as_ptr(),
        addr,
        buf_addr,
        count,
    );

    if (*gus).gf1.dma1 > 3 {
        if (*gus).gf1.enh_mode != 0 {
            address = addr >> 1;
        } else {
            if (addr & 0x1f) != 0 {
                dev_dbg(
                    (*(*gus).card).dev,
                    c"%s: unaligned address (0x%x)?\n".as_ptr(),
                    c"snd_gf1_dma_program".as_ptr(),
                    addr,
                );
                return;
            }
            address = (addr & 0x000c0000) | ((addr & 0x0003ffff) >> 1);
        }
    } else {
        address = addr;
    }

    dma_cmd = (SNDRV_GF1_DMA_ENABLE | (cmd as ::core::ffi::c_ushort as ::core::ffi::c_uint))
        as ::core::ffi::c_uchar;
    /*
     * Disabled in the original C source:
     * dma_cmd |= 0x08;
     */
    if (dma_cmd as ::core::ffi::c_uint & SNDRV_GF1_DMA_16BIT) != 0 {
        count = count.wrapping_add(1);
        count &= !1; /* align */
    }
    if (*gus).gf1.dma1 > 3 {
        dma_cmd = (dma_cmd as ::core::ffi::c_uint | SNDRV_GF1_DMA_WIDTH16) as ::core::ffi::c_uchar;
        count = count.wrapping_add(1);
        count &= !1; /* align */
    }
    snd_gf1_dma_ack(gus);
    snd_dma_program(
        (*gus).gf1.dma1,
        buf_addr,
        count,
        if (dma_cmd as ::core::ffi::c_uint & SNDRV_GF1_DMA_READ) != 0 {
            DMA_MODE_READ
        } else {
            DMA_MODE_WRITE
        },
    );
    /*
     * Disabled in the original C source:
     * dev_dbg(gus->card->dev,
     *         "address = 0x%x, count = 0x%x, dma_cmd = 0x%x\n",
     *         address << 1, count, dma_cmd);
     */
    let _guard = spinlock_irqsave_guard(&mut (*gus).reg_lock);
    if (*gus).gf1.enh_mode != 0 {
        address_high = ((address >> 16) & 0x000000f0) | (address & 0x0000000f);
        snd_gf1_write16(
            gus,
            SNDRV_GF1_GW_DRAM_DMA_LOW,
            (address >> 4) as ::core::ffi::c_ushort,
        );
        snd_gf1_write8(
            gus,
            SNDRV_GF1_GB_DRAM_DMA_HIGH,
            address_high as ::core::ffi::c_uchar,
        );
    } else {
        snd_gf1_write16(
            gus,
            SNDRV_GF1_GW_DRAM_DMA_LOW,
            (address >> 4) as ::core::ffi::c_ushort,
        );
    }
    snd_gf1_write8(gus, SNDRV_GF1_GB_DRAM_DMA_CONTROL, dma_cmd);
}

unsafe fn snd_gf1_dma_next_block(gus: *mut snd_gus_card) -> *mut snd_gf1_dma_block {
    let block: *mut snd_gf1_dma_block;

    /* PCM block have bigger priority than synthesizer one */
    if !(*gus).gf1.dma_data_pcm.is_null() {
        block = (*gus).gf1.dma_data_pcm;
        if (*gus).gf1.dma_data_pcm_last == block {
            (*gus).gf1.dma_data_pcm = ::core::ptr::null_mut();
            (*gus).gf1.dma_data_pcm_last = ::core::ptr::null_mut();
        } else {
            (*gus).gf1.dma_data_pcm = (*block).next;
        }
    } else if !(*gus).gf1.dma_data_synth.is_null() {
        block = (*gus).gf1.dma_data_synth;
        if (*gus).gf1.dma_data_synth_last == block {
            (*gus).gf1.dma_data_synth = ::core::ptr::null_mut();
            (*gus).gf1.dma_data_synth_last = ::core::ptr::null_mut();
        } else {
            (*gus).gf1.dma_data_synth = (*block).next;
        }
    } else {
        block = ::core::ptr::null_mut();
    }
    if !block.is_null() {
        (*gus).gf1.dma_ack = (*block).ack;
        (*gus).gf1.dma_private_data = (*block).private_data;
    }
    block
}

unsafe fn snd_gf1_dma_interrupt(gus: *mut snd_gus_card) {
    let block: *mut snd_gf1_dma_block;

    snd_gf1_dma_ack(gus);
    if let Some(dma_ack) = (*gus).gf1.dma_ack {
        dma_ack(gus, (*gus).gf1.dma_private_data);
    }
    {
        let _guard = spinlock_guard(&mut (*gus).dma_lock);
        if (*gus).gf1.dma_data_pcm.is_null() && (*gus).gf1.dma_data_synth.is_null() {
            (*gus).gf1.dma_ack = None;
            (*gus).gf1.dma_flags &= !SNDRV_GF1_DMA_TRIGGER;
            return;
        }
        block = snd_gf1_dma_next_block(gus);
    }
    if block.is_null() {
        return;
    }
    snd_gf1_dma_program(
        gus,
        (*block).addr,
        (*block).buf_addr,
        (*block).count,
        (*block).cmd as ::core::ffi::c_ushort as ::core::ffi::c_uint,
    );
    kfree(block as *const ::core::ffi::c_void);
    /*
     * Disabled in the original C source:
     * dev_dbg(gus->card->dev,
     *         "program dma (IRQ) - addr = 0x%x, buffer = 0x%lx, count = 0x%x, cmd = 0x%x\n",
     *         block->addr, block->buf_addr, block->count, block->cmd);
     */
}

pub unsafe extern "C" fn snd_gf1_dma_init(gus: *mut snd_gus_card) -> ::core::ffi::c_int {
    let _guard = mutex_guard(&mut (*gus).dma_mutex);
    (*gus).gf1.dma_shared += 1;
    if (*gus).gf1.dma_shared > 1 {
        return 0;
    }
    (*gus).gf1.interrupt_handler_dma_write = Some(snd_gf1_dma_interrupt);
    (*gus).gf1.dma_data_pcm = ::core::ptr::null_mut();
    (*gus).gf1.dma_data_pcm_last = ::core::ptr::null_mut();
    (*gus).gf1.dma_data_synth = ::core::ptr::null_mut();
    (*gus).gf1.dma_data_synth_last = ::core::ptr::null_mut();
    0
}

pub unsafe extern "C" fn snd_gf1_dma_done(gus: *mut snd_gus_card) -> ::core::ffi::c_int {
    let mut block: *mut snd_gf1_dma_block;

    let _guard = mutex_guard(&mut (*gus).dma_mutex);
    (*gus).gf1.dma_shared -= 1;
    if (*gus).gf1.dma_shared == 0 {
        snd_dma_disable((*gus).gf1.dma1);
        snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_DMA_WRITE);
        snd_gf1_dma_ack(gus);
        block = (*gus).gf1.dma_data_pcm;
        while !block.is_null() {
            (*gus).gf1.dma_data_pcm = (*block).next;
            kfree(block as *const ::core::ffi::c_void);
            block = (*gus).gf1.dma_data_pcm;
        }
        block = (*gus).gf1.dma_data_synth;
        while !block.is_null() {
            (*gus).gf1.dma_data_synth = (*block).next;
            kfree(block as *const ::core::ffi::c_void);
            block = (*gus).gf1.dma_data_synth;
        }
        (*gus).gf1.dma_data_pcm_last = ::core::ptr::null_mut();
        (*gus).gf1.dma_data_synth_last = ::core::ptr::null_mut();
    }
    0
}

pub unsafe extern "C" fn snd_gf1_dma_suspend(gus: *mut snd_gus_card) {
    let mut block: *mut snd_gf1_dma_block;

    let _guard = mutex_guard(&mut (*gus).dma_mutex);
    if (*gus).gf1.dma_shared == 0 {
        return;
    }

    snd_dma_disable((*gus).gf1.dma1);
    snd_gf1_dma_ack(gus);
    if let Some(dma_ack) = (*gus).gf1.dma_ack {
        dma_ack(gus, (*gus).gf1.dma_private_data);
    }
    (*gus).gf1.dma_ack = None;
    (*gus).gf1.dma_private_data = ::core::ptr::null_mut();

    block = (*gus).gf1.dma_data_pcm;
    while !block.is_null() {
        (*gus).gf1.dma_data_pcm = (*block).next;
        if let Some(ack) = (*block).ack {
            ack(gus, (*block).private_data);
        }
        kfree(block as *const ::core::ffi::c_void);
        block = (*gus).gf1.dma_data_pcm;
    }
    block = (*gus).gf1.dma_data_synth;
    while !block.is_null() {
        (*gus).gf1.dma_data_synth = (*block).next;
        if let Some(ack) = (*block).ack {
            ack(gus, (*block).private_data);
        }
        kfree(block as *const ::core::ffi::c_void);
        block = (*gus).gf1.dma_data_synth;
    }

    (*gus).gf1.dma_data_pcm_last = ::core::ptr::null_mut();
    (*gus).gf1.dma_data_synth_last = ::core::ptr::null_mut();
    (*gus).gf1.dma_flags &= !SNDRV_GF1_DMA_TRIGGER;
}

pub unsafe extern "C" fn snd_gf1_dma_transfer_block(
    gus: *mut snd_gus_card,
    __block: *mut snd_gf1_dma_block,
    atomic: ::core::ffi::c_int,
    synth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let block: *mut snd_gf1_dma_block;
    let mut free_block: *mut snd_gf1_dma_block = ::core::ptr::null_mut();

    block = kmalloc(
        ::core::mem::size_of::<snd_gf1_dma_block>(),
        if atomic != 0 { GFP_ATOMIC } else { GFP_KERNEL },
    ) as *mut snd_gf1_dma_block;
    if block.is_null() {
        return -ENOMEM;
    }

    *block = *__block;
    (*block).next = ::core::ptr::null_mut();

    dev_dbg(
        (*(*gus).card).dev,
        c"addr = 0x%x, buffer = 0x%lx, count = 0x%x, cmd = 0x%x\n".as_ptr(),
        (*block).addr,
        (*block).buffer as ::core::ffi::c_long,
        (*block).count,
        (*block).cmd,
    );

    dev_dbg(
        (*(*gus).card).dev,
        c"gus->gf1.dma_data_pcm_last = 0x%lx\n".as_ptr(),
        (*gus).gf1.dma_data_pcm_last as ::core::ffi::c_long,
    );
    dev_dbg(
        (*(*gus).card).dev,
        c"gus->gf1.dma_data_pcm = 0x%lx\n".as_ptr(),
        (*gus).gf1.dma_data_pcm as ::core::ffi::c_long,
    );

    {
        let _guard = spinlock_irqsave_guard(&mut (*gus).dma_lock);
        if synth != 0 {
            if !(*gus).gf1.dma_data_synth_last.is_null() {
                (*(*gus).gf1.dma_data_synth_last).next = block;
                (*gus).gf1.dma_data_synth_last = block;
            } else {
                (*gus).gf1.dma_data_synth = block;
                (*gus).gf1.dma_data_synth_last = block;
            }
        } else if !(*gus).gf1.dma_data_pcm_last.is_null() {
            (*(*gus).gf1.dma_data_pcm_last).next = block;
            (*gus).gf1.dma_data_pcm_last = block;
        } else {
            (*gus).gf1.dma_data_pcm = block;
            (*gus).gf1.dma_data_pcm_last = block;
        }
        if ((*gus).gf1.dma_flags & SNDRV_GF1_DMA_TRIGGER) == 0 {
            (*gus).gf1.dma_flags |= SNDRV_GF1_DMA_TRIGGER;
            free_block = snd_gf1_dma_next_block(gus);
        }
    }

    if !free_block.is_null() {
        snd_gf1_dma_program(
            gus,
            (*free_block).addr,
            (*free_block).buf_addr,
            (*free_block).count,
            (*free_block).cmd as ::core::ffi::c_ushort as ::core::ffi::c_uint,
        );
        kfree(free_block as *const ::core::ffi::c_void);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
