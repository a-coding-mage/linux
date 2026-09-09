/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2007 John Williams
 *
 * Reasonably optimised generic C-code for memcpy on Microblaze
 * This is generic C code to do efficient, alignment-aware memmove.
 *
 * It is based on demo code originally Copyright 2001 by Intel Corp, taken from
 * http://www.embedded.com/showArticle.jhtml?articleID=19205567
 *
 * Attempts were made, unsuccessfully, to contact the original
 * author of this code (Michael Morrow, Intel).  Below is the original
 * copyright notice.
 *
 * This software has been developed by Intel Corporation.
 * Intel specifically disclaims all warranties, express or
 * implied, and all liability, including consequential and all
 * other indirect damages, for the use of this program, including
 * liability for infringement of any proprietary rights,
 * and including the warranties of merchantability and fitness
 * for a particular purpose. Intel does not assume any
 * responsibility for and errors which may appear in this program
 * not any responsibility to update it.
 */

use core::ffi::c_void;

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, c: usize) -> *mut c_void;
}

#[cfg(CONFIG_OPT_LIB_FUNCTION)]
pub unsafe extern "C" fn memmove(
    v_dst: *mut c_void,
    v_src: *const c_void,
    mut c: usize,
) -> *mut c_void {
    let mut src = v_src as *const u8;
    let mut dst = v_dst as *mut u8;
    let mut i_src: *const u32;
    let mut i_dst: *mut u32;

    if c == 0 {
        return v_dst;
    }

    /* Use memcpy when source is higher than dest */
    if (v_dst as usize) <= (v_src as usize) {
        return memcpy(v_dst, v_src, c);
    }

    /* The following code tries to optimize the copy by using unsigned
     * alignment. This will work fine if both source and destination are
     * aligned on the same boundary. However, if they are aligned on
     * different boundaries shifts will be necessary. This might result in
     * bad performance on MicroBlaze systems without a barrel shifter.
     */
    /* FIXME this part needs more test */
    /* Do a descending copy - this is a bit trickier! */
    dst = dst.add(c);
    src = src.add(c);

    if c >= 4 {
        let mut value: u32;
        let mut buf_hold: u32;

        /* Align the destination to a word boundary. */
        /* This is done in an endian independent manner. */
        match (dst as usize) & 3 {
            3 => {
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
            }
            2 => {
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
            }
            1 => {
                dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1;
            }
            _ => {}
        }

        i_dst = dst as *mut u32;
        /* Choose a copy scheme based on the source */
        /* alignment relative to destination. */
        match (src as usize) & 3 {
            0 => {
                i_src = src as *const u32;
                while c >= 4 { i_dst = i_dst.sub(1); i_src = i_src.sub(1); *i_dst = *i_src; c -= 4; }
                src = i_src as *const u8;
            }
            1 => {
                i_src = (((src as usize + 4) & !3) as *const u32);
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    buf_hold = *i_src.sub(1) >> 24;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = (buf_hold << 8) | value; buf_hold = value >> 24; c -= 4; }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    buf_hold = (*i_src.sub(1) & 0xFF) << 24;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = buf_hold | ((value & 0xFFFFFF00) >> 8); buf_hold = (value & 0xFF) << 24; c -= 4; }
                }
                src = (i_src as *const u8).add(1);
            }
            2 => {
                i_src = (((src as usize + 4) & !3) as *const u32);
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    buf_hold = *i_src.sub(1) >> 16;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = (buf_hold << 16) | value; buf_hold = value >> 16; c -= 4; }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    buf_hold = (*i_src.sub(1) & 0xFFFF) << 16;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = buf_hold | ((value & 0xFFFF0000) >> 16); buf_hold = (value & 0xFFFF) << 16; c -= 4; }
                }
                src = (i_src as *const u8).add(2);
            }
            3 => {
                i_src = (((src as usize + 4) & !3) as *const u32);
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    buf_hold = *i_src.sub(1) >> 8;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = (buf_hold << 24) | value; buf_hold = value >> 8; c -= 4; }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    buf_hold = (*i_src.sub(1) & 0xFFFFFF) << 8;
                    while c >= 4 { i_src = i_src.sub(1); value = *i_src; i_dst = i_dst.sub(1); *i_dst = buf_hold | ((value & 0xFF000000) >> 24); buf_hold = (value & 0xFFFFFF) << 8; c -= 4; }
                }
                src = (i_src as *const u8).add(3);
            }
            _ => unreachable!(),
        }
        dst = i_dst as *mut u8;
    }

    /* simple fast copy, ... unless a cache boundary is crossed */
    /* Finish off any remaining bytes */
    match c {
        4 => { dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; }
        3 => { dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; }
        2 => { dst = dst.sub(1); src = src.sub(1); *dst = *src; c -= 1; dst = dst.sub(1); src = src.sub(1); *dst = *src; }
        1 => { dst = dst.sub(1); src = src.sub(1); *dst = *src; }
        _ => {}
    }
    v_dst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
