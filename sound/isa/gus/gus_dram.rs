// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  DRAM access routines
 */

use core::ffi::{c_char, c_int};

use crate::*;

unsafe fn snd_gus_dram_poke(
    gus: *mut snd_gus_card,
    mut _buffer: *mut c_char,
    mut address: u32,
    mut size: u32,
) -> c_int {
    let mut size1: u32;
    let mut size2: u32;
    let mut buffer: [c_char; 256] = [0; 256];
    let mut pbuffer: *mut c_char;

    while size > 0 {
        size1 = if size > buffer.len() as u32 {
            buffer.len() as u32
        } else {
            size
        };
        if copy_from_user(buffer.as_mut_ptr() as *mut _, _buffer as *const _, size1 as usize) != 0 {
            return -EFAULT;
        }
        if (*gus).interwave != 0 {
            // C source used guard(spinlock_irqsave)(&gus->reg_lock) for this scope.
            let _guard = spinlock_irqsave_guard(&mut (*gus).reg_lock);
            snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x01);
            snd_gf1_dram_addr(gus, address);
            outsb(GUSP(gus, DRAM), buffer.as_ptr() as *const _, size1);
            address = address.wrapping_add(size1);
        } else {
            pbuffer = buffer.as_mut_ptr();
            size2 = size1;
            while size2 != 0 {
                size2 = size2.wrapping_sub(1);
                snd_gf1_poke(gus, address, *pbuffer);
                address = address.wrapping_add(1);
                pbuffer = pbuffer.add(1);
            }
        }
        size = size.wrapping_sub(size1);
        _buffer = _buffer.add(size1 as usize);
    }
    0
}

pub unsafe fn snd_gus_dram_write(
    gus: *mut snd_gus_card,
    buffer: *mut c_char,
    address: u32,
    size: u32,
) -> c_int {
    snd_gus_dram_poke(gus, buffer, address, size)
}

unsafe fn snd_gus_dram_peek(
    gus: *mut snd_gus_card,
    mut _buffer: *mut c_char,
    mut address: u32,
    mut size: u32,
    rom: c_int,
) -> c_int {
    let mut size1: u32;
    let mut size2: u32;
    let mut buffer: [c_char; 256] = [0; 256];
    let mut pbuffer: *mut c_char;

    while size > 0 {
        size1 = if size > buffer.len() as u32 {
            buffer.len() as u32
        } else {
            size
        };
        if (*gus).interwave != 0 {
            // C source used guard(spinlock_irqsave)(&gus->reg_lock) for this scope.
            let _guard = spinlock_irqsave_guard(&mut (*gus).reg_lock);
            snd_gf1_write8(
                gus,
                SNDRV_GF1_GB_MEMORY_CONTROL,
                if rom != 0 { 0x03 } else { 0x01 },
            );
            snd_gf1_dram_addr(gus, address);
            insb(GUSP(gus, DRAM), buffer.as_mut_ptr() as *mut _, size1);
            snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x01);
            address = address.wrapping_add(size1);
        } else {
            pbuffer = buffer.as_mut_ptr();
            size2 = size1;
            while size2 != 0 {
                size2 = size2.wrapping_sub(1);
                *pbuffer = snd_gf1_peek(gus, address);
                pbuffer = pbuffer.add(1);
                address = address.wrapping_add(1);
            }
        }
        if copy_to_user(_buffer as *mut _, buffer.as_ptr() as *const _, size1 as usize) != 0 {
            return -EFAULT;
        }
        size = size.wrapping_sub(size1);
        _buffer = _buffer.add(size1 as usize);
    }
    0
}

pub unsafe fn snd_gus_dram_read(
    gus: *mut snd_gus_card,
    buffer: *mut c_char,
    address: u32,
    size: u32,
    rom: c_int,
) -> c_int {
    snd_gus_dram_peek(gus, buffer, address, size, rom)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
