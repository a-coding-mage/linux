// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/sysv68.c
 *
 *  Copyright (C) 2007 Philippe De Muyter <phdm@macqel.be>
 */

// Dependency declarations supplied by check.h are intentionally external.

/*
 *	Volume ID structure: on first 256-bytes sector of disk
 */
#[repr(C)]
pub struct volumeid {
    pub vid_unused: [u8; 248],
    pub vid_mac: [u8; 8], /* ASCII string "MOTOROLA" */
}

/*
 *	config block: second 256-bytes sector on disk
 */
#[repr(C)]
pub struct dkconfig {
    pub ios_unused0: [u8; 128],
    pub ios_slcblk: __be32, /* Slice table block number */
    pub ios_slccnt: __be16, /* Number of entries in slice table */
    pub ios_unused1: [u8; 122],
}

/*
 *	combined volumeid and dkconfig block
 */
#[repr(C)]
pub struct dkblk0 {
    pub dk_vid: volumeid,
    pub dk_ios: dkconfig,
}

/*
 *	Slice Table Structure
 */
#[repr(C)]
pub struct slice {
    pub nblocks: __be32, /* slice size (in blocks) */
    pub blkoff: __be32,  /* block offset of slice */
}

pub unsafe fn sysv68_partition(state: *mut parsed_partitions) -> i32 {
    let mut i: i32;
    let mut slices: i32;
    let mut slot: i32 = 1;
    let mut sect: Sector = core::mem::zeroed();
    let mut data: *mut u8;
    let mut b: *mut dkblk0;
    let mut slice_ptr: *mut slice;

    data = read_part_sector(state, 0, &mut sect);
    if data.is_null() {
        return -1;
    }

    b = data as *mut dkblk0;
    if (*b).dk_vid.vid_mac != *b"MOTOROLA" {
        put_dev_sector(sect);
        return 0;
    }
    slices = be16_to_cpu((*b).dk_ios.ios_slccnt) as i32;
    i = be32_to_cpu((*b).dk_ios.ios_slcblk) as i32;
    put_dev_sector(sect);

    data = read_part_sector(state, i, &mut sect);
    if data.is_null() {
        return -1;
    }

    slices -= 1; /* last slice is the whole disk */
    seq_buf_printf(
        &mut (*state).pp_buf,
        "sysV68: %s(s%u)",
        (*state).name,
        slices as u32,
    );
    slice_ptr = data as *mut slice;
    i = 0;
    while i < slices {
        if slot == (*state).limit {
            break;
        }
        if be32_to_cpu((*slice_ptr).nblocks) != 0 {
            put_partition(
                state,
                slot,
                be32_to_cpu((*slice_ptr).blkoff),
                be32_to_cpu((*slice_ptr).nblocks),
            );
            seq_buf_printf(&mut (*state).pp_buf, "(s%u)", i as u32);
        }
        slot += 1;
        i += 1;
        slice_ptr = slice_ptr.add(1);
    }
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    put_dev_sector(sect);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
