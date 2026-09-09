// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/ultrix.c
 *
 *  Code extracted from drivers/block/genhd.c
 *
 *  Re-organised Jul 1999 Russell King
 */

// #include "check.h"

use core::ffi::c_char;

#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parsed_partitions {
    pub pp_buf: seq_buf,
}

pub type Sector = u64;

extern "C" {
    fn read_part_sector(
        state: *mut parsed_partitions,
        n: u64,
        p: *mut Sector,
    ) -> *mut u8;
    fn put_partition(
        state: *mut parsed_partitions,
        part: i32,
        from: u32,
        size: i32,
    );
    fn put_dev_sector(sect: Sector);
    fn seq_buf_puts(buf: *mut seq_buf, s: *const c_char);
}

#[repr(C)]
struct PtInfo {
    pi_nblocks: i32,
    pi_blkoff: u32,
}

#[repr(C)]
struct UltrixDisklabel {
    pt_magic: i32,
    pt_valid: i32,
    pt_part: [PtInfo; 8],
}

const PT_MAGIC: i32 = 0x032957;
const PT_VALID: i32 = 1;

pub unsafe fn ultrix_partition(state: *mut parsed_partitions) -> i32 {
    let mut i: i32;
    let mut sect: Sector = 0;
    let data: *mut u8;

    data = read_part_sector(
        state,
        ((16384usize - core::mem::size_of::<UltrixDisklabel>()) / 512) as u64,
        &mut sect,
    );
    if data.is_null() {
        return -1;
    }

    let label = (data.add(512 - core::mem::size_of::<UltrixDisklabel>()))
        as *mut UltrixDisklabel;

    if (*label).pt_magic == PT_MAGIC && (*label).pt_valid == PT_VALID {
        i = 0;
        while i < 8 {
            if (*label).pt_part[i as usize].pi_nblocks != 0 {
                put_partition(
                    state,
                    i + 1,
                    (*label).pt_part[i as usize].pi_blkoff,
                    (*label).pt_part[i as usize].pi_nblocks,
                );
            }
            i += 1;
        }
        put_dev_sector(sect);
        seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr() as *const c_char);
        return 1;
    } else {
        put_dev_sector(sect);
        return 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
