/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2007 John Williams
 *
 * Reasonably optimised generic C-code for memcpy on Microblaze
 * This is generic C code to do efficient, alignment-aware memcpy.
 */

// The original implementation is compiled only when CONFIG_OPT_LIB_FUNCTION
// is enabled. The Linux headers supplied the source and destination types,
// size type, and the likely/fallthrough annotations.
#[cfg(CONFIG_OPT_LIB_FUNCTION)]
pub unsafe extern "C" fn memcpy(v_dst: *mut core::ffi::c_void, v_src: *const core::ffi::c_void, mut c: usize) -> *mut core::ffi::c_void {
    let mut src = v_src as *const u8;
    let mut dst = v_dst as *mut u8;

    if c >= 4 {
        let mut i_src: *const u32;
        let mut i_dst: *mut u32;

        // Align the destination to a word boundary.
        match (dst as usize) & 3 {
            1 => {
                *dst = *src;
                dst = dst.add(1);
                src = src.add(1);
                c -= 1;
            }
            2 => {
                *dst = *src;
                dst = dst.add(1);
                src = src.add(1);
                c -= 1;
            }
            3 => {
                *dst = *src;
                dst = dst.add(1);
                src = src.add(1);
                c -= 1;
            }
            _ => {}
        }

        i_dst = dst as *mut u32;

        match (src as usize) & 3 {
            0 => {
                i_src = src as *const u32;
                while c >= 4 {
                    *i_dst = *i_src;
                    i_dst = i_dst.add(1);
                    i_src = i_src.add(1);
                    c -= 4;
                }
                src = i_src as *const u8;
            }
            1 => {
                i_src = ((src as usize) & !3) as *const u32;
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    let mut buf_hold = (*i_src).wrapping_shl(8);
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | value.wrapping_shr(24);
                        i_dst = i_dst.add(1);
                        i_src = i_src.add(1);
                        buf_hold = value.wrapping_shl(8);
                        c -= 4;
                    }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    let mut buf_hold = (*i_src & 0xFFFFFF00) >> 8;
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | ((value & 0xFF) << 24);
                        i_dst = i_dst.add(1);
                        i_src = i_src.add(1);
                        buf_hold = (value & 0xFFFFFF00) >> 8;
                        c -= 4;
                    }
                }
                src = (i_src as *const u8).sub(3);
            }
            2 => {
                i_src = ((src as usize) & !3) as *const u32;
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    let mut buf_hold = (*i_src).wrapping_shl(16);
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | value.wrapping_shr(16);
                        i_dst = i_dst.add(1); i_src = i_src.add(1);
                        buf_hold = value.wrapping_shl(16); c -= 4;
                    }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    let mut buf_hold = (*i_src & 0xFFFF0000) >> 16;
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | ((value & 0xFFFF) << 16);
                        i_dst = i_dst.add(1); i_src = i_src.add(1);
                        buf_hold = (value & 0xFFFF0000) >> 16; c -= 4;
                    }
                }
                src = (i_src as *const u8).sub(2);
            }
            3 => {
                i_src = ((src as usize) & !3) as *const u32;
                #[cfg(not(__MICROBLAZEEL__))]
                {
                    let mut buf_hold = (*i_src).wrapping_shl(24);
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | value.wrapping_shr(8);
                        i_dst = i_dst.add(1); i_src = i_src.add(1);
                        buf_hold = value.wrapping_shl(24); c -= 4;
                    }
                }
                #[cfg(__MICROBLAZEEL__)]
                {
                    let mut buf_hold = (*i_src & 0xFF000000) >> 24;
                    i_src = i_src.add(1);
                    while c >= 4 {
                        let value = *i_src;
                        *i_dst = buf_hold | ((value & 0xFFFFFF) << 8);
                        i_dst = i_dst.add(1); i_src = i_src.add(1);
                        buf_hold = (value & 0xFF000000) >> 24; c -= 4;
                    }
                }
                src = (i_src as *const u8).sub(1);
            }
            _ => unreachable!(),
        }
        dst = i_dst as *mut u8;
    }

    while c > 0 {
        *dst = *src;
        dst = dst.add(1);
        src = src.add(1);
        c -= 1;
    }
    v_dst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
