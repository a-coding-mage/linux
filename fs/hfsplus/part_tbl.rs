/*
 * linux/fs/hfsplus/part_tbl.c
 *
 * Copyright (C) 1996-1997  Paul H. Hargrove
 * This file may be distributed under the terms of
 * the GNU General Public License.
 *
 * Original code to handle the new style Mac partition table based on
 * a patch contributed by Holger Schemel (aeglos@valinor.owl.de).
 *
 * In function preconditions the term "valid" applied to a pointer to
 * a structure means that the pointer is non-NULL and the structure it
 * points to has all fields initialized to consistent values.
 *
 */

/* Dependencies: linux/slab.h and hfsplus_fs.h */

/* offsets to various blocks */
const HFS_DD_BLK: u32 = 0; /* Driver Descriptor block */
const HFS_PMAP_BLK: u32 = 1; /* First block of partition map */
const HFS_MDB_BLK: u32 = 2; /* Block (w/i partition) of MDB */

/* magic numbers for various disk blocks */
const HFS_DRVR_DESC_MAGIC: u16 = 0x4552; /* "ER": driver descriptor map */
const HFS_OLD_PMAP_MAGIC: u16 = 0x5453; /* "TS": old-type partition map */
const HFS_NEW_PMAP_MAGIC: u16 = 0x504d; /* "PM": new-type partition map */
const HFS_SUPER_MAGIC: u16 = 0x4244; /* "BD": HFS MDB (super block) */
const HFS_MFS_SUPER_MAGIC: u16 = 0xd2d7; /* MFS MDB (super block) */

#[repr(C, packed)]
pub struct new_pmap {
    pub pmSig: __be16,
    pub reSigPad: __be16,
    pub pmMapBlkCnt: __be32,
    pub pmPyPartStart: __be32,
    pub pmPartBlkCnt: __be32,
    pub pmPartName: [u8; 32],
    pub pmPartType: [u8; 32],
}

#[repr(C, packed)]
pub struct old_pmap_entry {
    pub pdStart: __be32,
    pub pdSize: __be32,
    pub pdFSID: __be32,
}

#[repr(C, packed)]
pub struct old_pmap {
    pub pdSig: __be16,
    pub pdEntry: [old_pmap_entry; 42],
}

unsafe fn hfs_parse_old_pmap(
    sb: *mut super_block,
    pm: *mut old_pmap,
    part_start: *mut sector_t,
    part_size: *mut sector_t,
) -> c_int {
    let sbi = HFSPLUS_SB(sb);

    for i in 0..42 {
        let p: *mut old_pmap_entry = &mut (*pm).pdEntry[i];

        if (*p).pdStart != 0
            && (*p).pdSize != 0
            && (*p).pdFSID == cpu_to_be32(0x54465331) /* "TFS1" */
            && ((*sbi).part < 0 || (*sbi).part == i as _) {
            *part_start += be32_to_cpu((*p).pdStart) as _;
            *part_size = be32_to_cpu((*p).pdSize) as _;
            return 0;
        }
    }

    -ENOENT
}

unsafe fn hfs_parse_new_pmap(
    sb: *mut super_block,
    buf: *mut c_void,
    mut pm: *mut new_pmap,
    part_start: *mut sector_t,
    part_size: *mut sector_t,
) -> c_int {
    let sbi = HFSPLUS_SB(sb);
    let size = be32_to_cpu((*pm).pmMapBlkCnt) as c_int;
    let buf_size = hfsplus_min_io_size(sb);
    let mut i: c_int = 0;

    loop {
        if memcmp((*pm).pmPartType.as_ptr() as *const c_void,
                  b"Apple_HFS\0".as_ptr() as *const c_void, 9) == 0
            && ((*sbi).part < 0 || (*sbi).part == i as _) {
            *part_start += be32_to_cpu((*pm).pmPyPartStart) as _;
            *part_size = be32_to_cpu((*pm).pmPartBlkCnt) as _;
            return 0;
        }

        i += 1;
        if i >= size {
            return -ENOENT;
        }

        pm = ((pm as *mut u8).add(HFSPLUS_SECTOR_SIZE as usize)) as *mut new_pmap;
        if (pm as *mut u8).offset_from(buf as *mut u8) as usize >= buf_size as usize {
            let mut next: *mut c_void = pm as *mut c_void;
            let res = hfsplus_submit_bio(
                sb, *part_start + HFS_PMAP_BLK as _, buf, &mut next, REQ_OP_READ,
            );
            if res != 0 {
                return res;
            }
            pm = next as *mut new_pmap;
        }

        if (*pm).pmSig != cpu_to_be16(HFS_NEW_PMAP_MAGIC) {
            return -ENOENT;
        }
    }
}

/*
 * Parse the partition map looking for the start and length of a
 * HFS/HFS+ partition.
 */
pub unsafe fn hfs_part_find(
    sb: *mut super_block,
    part_start: *mut sector_t,
    part_size: *mut sector_t,
) -> c_int {
    let buf = kmalloc(hfsplus_min_io_size(sb), GFP_KERNEL);
    if buf.is_null() {
        return -ENOMEM;
    }

    let mut data: *mut c_void = core::ptr::null_mut();
    let res = hfsplus_submit_bio(
        sb, *part_start + HFS_PMAP_BLK as _, buf, &mut data, REQ_OP_READ,
    );
    if res != 0 {
        kfree(buf);
        return res;
    }

    let res = match be16_to_cpu(*(data as *mut __be16)) {
        HFS_OLD_PMAP_MAGIC => hfs_parse_old_pmap(sb, data as *mut old_pmap, part_start, part_size),
        HFS_NEW_PMAP_MAGIC => hfs_parse_new_pmap(sb, buf, data as *mut new_pmap, part_start, part_size),
        _ => -ENOENT,
    };
    kfree(buf);
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
