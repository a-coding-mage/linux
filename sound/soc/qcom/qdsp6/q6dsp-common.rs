// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Rust translation of q6dsp-common.c.
// Original dependencies:
//   q6dsp-common.h
//   linux/kernel.h
//   linux/module.h
//   linux/string.h
//   linux/errno.h

use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn q6dsp_map_channels(ch_map: *mut u8, ch: i32) -> i32 {
    unsafe {
        ptr::write_bytes(ch_map, 0, PCM_MAX_NUM_CHANNEL as usize);
    }

    match ch {
        1 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FC;
            }
        }
        2 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
            }
        }
        3 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
                *ch_map.add(2) = PCM_CHANNEL_FC;
            }
        }
        4 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
                *ch_map.add(2) = PCM_CHANNEL_LS;
                *ch_map.add(3) = PCM_CHANNEL_RS;
            }
        }
        5 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
                *ch_map.add(2) = PCM_CHANNEL_FC;
                *ch_map.add(3) = PCM_CHANNEL_LS;
                *ch_map.add(4) = PCM_CHANNEL_RS;
            }
        }
        6 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
                *ch_map.add(2) = PCM_CHANNEL_LFE;
                *ch_map.add(3) = PCM_CHANNEL_FC;
                *ch_map.add(4) = PCM_CHANNEL_LS;
                *ch_map.add(5) = PCM_CHANNEL_RS;
            }
        }
        8 => {
            unsafe {
                *ch_map.add(0) = PCM_CHANNEL_FL;
                *ch_map.add(1) = PCM_CHANNEL_FR;
                *ch_map.add(2) = PCM_CHANNEL_LFE;
                *ch_map.add(3) = PCM_CHANNEL_FC;
                *ch_map.add(4) = PCM_CHANNEL_LS;
                *ch_map.add(5) = PCM_CHANNEL_RS;
                *ch_map.add(6) = PCM_CHANNEL_LB;
                *ch_map.add(7) = PCM_CHANNEL_RB;
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}
// EXPORT_SYMBOL_GPL(q6dsp_map_channels);

#[no_mangle]
pub extern "C" fn q6dsp_get_channel_allocation(channels: i32) -> i32 {
    let channel_allocation: i32;

    /* HDMI spec CEA-861-E: Table 28 Audio InfoFrame Data Byte 4 */
    match channels {
        2 => {
            channel_allocation = 0;
        }
        3 => {
            channel_allocation = 0x02;
        }
        4 => {
            channel_allocation = 0x06;
        }
        5 => {
            channel_allocation = 0x0A;
        }
        6 => {
            channel_allocation = 0x0B;
        }
        7 => {
            channel_allocation = 0x12;
        }
        8 => {
            channel_allocation = 0x13;
        }
        _ => {
            return -EINVAL;
        }
    }

    channel_allocation
}
// EXPORT_SYMBOL_GPL(q6dsp_get_channel_allocation);

// MODULE_DESCRIPTION("ASoC MSM QDSP6 helper functions");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
