// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/karma.c
 *  Rio Karma partition info.
 *
 *  Copyright (C) 2006 Bob Copeland (me@bobcopeland.com)
 *  based on osf.c
 */

use core::ptr;

// Dependency supplied by the surrounding partitioning code.
#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parsed_partitions {
    pub limit: i32,
    pub pp_buf: seq_buf,
}

pub type Sector = u64;

extern "C" {
    fn read_part_sector(
        state: *mut parsed_partitions,
        n: u64,
        p: *mut Sector,
    ) -> *mut u8;
    fn put_dev_sector(sect: Sector);
    fn put_partition(state: *mut parsed_partitions, slot: i32, from: u32, size: u32);
    fn seq_buf_puts(buf: *mut seq_buf, s: *const u8);
    fn le16_to_cpu(v: u16) -> u16;
    fn le32_to_cpu(v: u32) -> u32;
}

pub const KARMA_LABEL_MAGIC: u16 = 0xAB56;

#[repr(C, packed)]
struct DPartition {
    p_res: u32,
    p_fstype: u8,
    p_res2: [u8; 3],
    p_offset: u32,
    p_size: u32,
}

#[repr(C, packed)]
struct Disklabel {
    d_reserved: [u8; 270],
    d_partitions: [DPartition; 2],
    d_blank: [u8; 208],
    d_magic: u16,
}

pub unsafe fn karma_partition(state: *mut parsed_partitions) -> i32 {
    let mut i: i32;
    let mut slot: i32 = 1;
    let mut sect: Sector = 0;
    let data: *mut u8;
    let label: *mut Disklabel;
    let mut p: *mut DPartition;

    data = read_part_sector(state, 0, &mut sect);
    if data.is_null() {
        return -1;
    }

    label = data as *mut Disklabel;
    if le16_to_cpu(ptr::read_unaligned(ptr::addr_of!((*label).d_magic))) != KARMA_LABEL_MAGIC {
        put_dev_sector(sect);
        return 0;
    }

    p = ptr::addr_of_mut!((*label).d_partitions) as *mut DPartition;
    i = 0;
    while i < 2 {
        if slot == (*state).limit {
            break;
        }

        let fstype = ptr::read_unaligned(ptr::addr_of!((*p).p_fstype));
        let size = le32_to_cpu(ptr::read_unaligned(ptr::addr_of!((*p).p_size)));
        if fstype == 0x4d && size != 0 {
            let offset = le32_to_cpu(ptr::read_unaligned(ptr::addr_of!((*p).p_offset)));
            put_partition(state, slot, offset, size);
        }
        slot += 1;
        i += 1;
        p = p.add(1);
    }
    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr());
    put_dev_sector(sect);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
