/*
 *  linux/fs/hfs/part_tbl.c
 *
 * Copyright (C) 1996-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * Original code to handle the new style Mac partition table based on
 * a patch contributed by Holger Schemel (aeglos@valinor.owl.de).
 */

// Dependency declarations from hfs_fs.h are supplied by the surrounding code.

/*
 * The new style Mac partition map
 *
 * For each partition on the media there is a physical block (512-byte
 * block) containing one of these structures.  These blocks are
 * contiguous starting at block 1.
 */
#[repr(C, packed)]
pub struct new_pmap {
    pub pmSig: __be16,        /* signature */
    pub reSigPad: __be16,     /* padding */
    pub pmMapBlkCnt: __be32,  /* partition blocks count */
    pub pmPyPartStart: __be32,/* physical block start of partition */
    pub pmPartBlkCnt: __be32, /* physical block count of partition */
    pub pmPartName: [u8; 32], /* (null terminated?) string
                                 giving the name of this partition */
    pub pmPartType: [u8; 32], /* (null terminated?) string
                                 giving the type of this partition */
    /* a bunch more stuff we don't need */
}

/*
 * The old style Mac partition map
 *
 * The partition map consists for a 2-byte signature followed by an
 * array of these structures.  The map is terminated with an all-zero
 * one of these.
 */
#[repr(C, packed)]
pub struct old_pmap {
    pub pdSig: __be16, /* Signature bytes */
    pub pdEntry: [old_pmap_entry; 42],
}

#[repr(C, packed)]
pub struct old_pmap_entry {
    pub pdStart: __be32,
    pub pdSize: __be32,
    pub pdFSID: __be32,
}

/*
 * hfs_part_find()
 *
 * Parse the partition map looking for the
 * start and length of the 'part'th HFS partition.
 */
pub unsafe fn hfs_part_find(
    sb: *mut super_block,
    part_start: *mut sector_t,
    part_size: *mut sector_t,
) -> i32 {
    let mut bh: *mut buffer_head;
    let data: *mut __be16 = core::ptr::null_mut();
    let mut i: i32;
    let mut size: i32;
    let mut res: i32;

    res = -ENOENT;
    bh = sb_bread512(sb, *part_start + HFS_PMAP_BLK, data);
    if bh.is_null() {
        return -EIO;
    }

    match be16_to_cpu(*( (*bh).b_data as *mut __be16)) {
        HFS_OLD_PMAP_MAGIC => {
            let pm = (*bh).b_data as *mut old_pmap;
            let mut p = (*pm).pdEntry.as_mut_ptr();
            size = 42;
            i = 0;
            while i < size {
                if (*p).pdStart != 0
                    && (*p).pdSize != 0
                    && (*p).pdFSID == cpu_to_be32(0x54465331) /* "TFS1" */
                    && (HFS_SB(sb).part < 0 || HFS_SB(sb).part == i)
                {
                    *part_start += be32_to_cpu((*p).pdStart) as sector_t;
                    *part_size = be32_to_cpu((*p).pdSize) as sector_t;
                    res = 0;
                }
                p = p.add(1);
                i += 1;
            }
        }
        HFS_NEW_PMAP_MAGIC => {
            let mut pm = (*bh).b_data as *mut new_pmap;
            size = be32_to_cpu((*pm).pmMapBlkCnt) as i32;
            i = 0;
            while i < size {
                if libc::memcmp((*pm).pmPartType.as_ptr() as *const _, b"Apple_HFS\0".as_ptr() as *const _, 9) == 0
                    && (HFS_SB(sb).part < 0 || HFS_SB(sb).part == i)
                {
                    *part_start += be32_to_cpu((*pm).pmPyPartStart) as sector_t;
                    *part_size = be32_to_cpu((*pm).pmPartBlkCnt) as sector_t;
                    res = 0;
                    break;
                }
                brelse(bh);
                i += 1;
                bh = sb_bread512(sb, *part_start + HFS_PMAP_BLK + i as sector_t, pm as *mut __be16);
                if bh.is_null() {
                    return -EIO;
                }
                if (*pm).pmSig != cpu_to_be16(HFS_NEW_PMAP_MAGIC) {
                    break;
                }
            }
        }
        _ => {}
    }
    brelse(bh);

    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
