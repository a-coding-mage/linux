// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/osf.c
 *
 *  Code extracted from drivers/block/genhd.c
 *
 *  Copyright (C) 1991-1998  Linus Torvalds
 *  Re-organised Feb 1998 Russell King
 */

use core::ffi::c_int;

pub const MAX_OSF_PARTITIONS: usize = 18;
pub const DISKLABELMAGIC: u32 = 0x82564557;

pub type Sector = u64;
pub type __le16 = u16;
pub type __le32 = u32;
pub type u8 = u8;

#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parsed_partitions {
    pub pp_buf: seq_buf,
    pub limit: c_int,
}

extern "C" {
    fn read_part_sector(
        state: *mut parsed_partitions,
        n: Sector,
        p: *mut Sector,
    ) -> *mut u8;
    fn put_dev_sector(sect: Sector);
    fn put_partition(
        state: *mut parsed_partitions,
        slot: c_int,
        from: u32,
        size: u32,
    );
    fn seq_buf_puts(buf: *mut seq_buf, s: *const u8);
    fn le32_to_cpu(v: __le32) -> u32;
    fn le16_to_cpu(v: __le16) -> u16;
}

#[repr(C)]
pub struct d_partition {
    pub p_size: __le32,
    pub p_offset: __le32,
    pub p_fsize: __le32,
    pub p_fstype: u8,
    pub p_frag: u8,
    pub p_cpg: __le16,
}

#[repr(C)]
pub struct disklabel {
    pub d_magic: __le32,
    pub d_type: __le16,
    pub d_subtype: __le16,
    pub d_typename: [u8; 16],
    pub d_packname: [u8; 16],
    pub d_secsize: __le32,
    pub d_nsectors: __le32,
    pub d_ntracks: __le32,
    pub d_ncylinders: __le32,
    pub d_secpercyl: __le32,
    pub d_secprtunit: __le32,
    pub d_sparespertrack: __le16,
    pub d_sparespercyl: __le16,
    pub d_acylinders: __le32,
    pub d_rpm: __le16,
    pub d_interleave: __le16,
    pub d_trackskew: __le16,
    pub d_cylskew: __le16,
    pub d_headswitch: __le32,
    pub d_trkseek: __le32,
    pub d_flags: __le32,
    pub d_drivedata: [__le32; 5],
    pub d_spare: [__le32; 5],
    pub d_magic2: __le32,
    pub d_checksum: __le16,
    pub d_npartitions: __le16,
    pub d_bbsize: __le32,
    pub d_sbsize: __le32,
    pub d_partitions: [d_partition; MAX_OSF_PARTITIONS],
}

pub unsafe extern "C" fn osf_partition(state: *mut parsed_partitions) -> c_int {
    let mut slot: c_int = 1;
    let npartitions: u16;
    let mut sect: Sector = 0;

    let data = read_part_sector(state, 0, &mut sect);
    if data.is_null() {
        return -1;
    }

    let label = (data.add(64)) as *mut disklabel;
    let mut partition = (*label).d_partitions.as_mut_ptr();
    if le32_to_cpu((*label).d_magic) != DISKLABELMAGIC {
        put_dev_sector(sect);
        return 0;
    }
    if le32_to_cpu((*label).d_magic2) != DISKLABELMAGIC {
        put_dev_sector(sect);
        return 0;
    }
    npartitions = le16_to_cpu((*label).d_npartitions);
    if npartitions as usize > MAX_OSF_PARTITIONS {
        put_dev_sector(sect);
        return 0;
    }
    for _i in 0..npartitions {
        if slot == (*state).limit {
            break;
        }
        if le32_to_cpu((*partition).p_size) != 0 {
            put_partition(
                state,
                slot,
                le32_to_cpu((*partition).p_offset),
                le32_to_cpu((*partition).p_size),
            );
        }
        slot += 1;
        partition = partition.add(1);
    }
    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr());
    put_dev_sector(sect);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
