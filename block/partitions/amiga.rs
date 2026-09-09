// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/amiga.c
 *
 *  Code extracted from drivers/block/genhd.c
 *
 *  Copyright (C) 1991-1998  Linus Torvalds
 *  Re-organised Feb 1998 Russell King
 */

// Dependency equivalents supplied by the surrounding kernel translation.

/* magic offsets in partition DosEnvVec */
const NR_HD: usize = 3;
const NR_SECT: usize = 5;
const LO_CYL: usize = 9;
const HI_CYL: usize = 10;

#[inline]
unsafe fn checksum_block(mut m: *const __be32, mut size: i32) -> u32 {
    let mut sum: u32 = 0;
    while size != 0 {
        sum = sum.wrapping_add(be32_to_cpu(*m));
        m = m.add(1);
        size -= 1;
    }
    sum
}

pub unsafe fn amiga_partition(state: *mut parsed_partitions) -> i32 {
    let mut sect: Sector = core::mem::MaybeUninit::uninit().assume_init();
    let mut data: *mut u8;
    let mut rdb: *mut RigidDiskBlock;
    let mut pb: *mut PartitionBlock;
    let mut start_sect: u64;
    let mut nr_sects: u64;
    let mut blk: sector_t;
    let mut end_sect: sector_t;
    let mut cylblk: u32; // rdb_CylBlocks = nr_heads*sect_per_track
    let mut nr_hd: u32;
    let mut nr_sect: u32;
    let mut lo_cyl: u32;
    let mut hi_cyl: u32;
    let mut part: i32;
    let mut res: i32 = 0;
    let mut blksize: u32 = 1; // Multiplier for disk block size
    let mut slot: i32 = 1;

    blk = 0;
    loop {
        if blk == RDB_ALLOCATION_LIMIT {
            break;
        }
        data = read_part_sector(state, blk, &mut sect);
        if data.is_null() {
            pr_err!("Dev {}: unable to read RDB block {}\n", (*(*state).disk).disk_name, blk);
            res = -1;
            break;
        }
        if *(data as *const __be32) != cpu_to_be32(IDNAME_RIGIDDISK) {
            put_dev_sector(sect);
            blk = blk.wrapping_add(1);
            continue;
        }

        rdb = data as *mut RigidDiskBlock;
        if checksum_block(data as *const __be32,
                          (be32_to_cpu((*rdb).rdb_SummedLongs) & 0x7f) as i32) == 0 {
            break;
        }
        // Try again with 0xdc..0xdf zeroed, Windows might have trashed it.
        *(data.add(0xdc) as *mut __be32) = 0;
        if checksum_block(data as *const __be32,
                          (be32_to_cpu((*rdb).rdb_SummedLongs) & 0x7f) as i32) == 0 {
            pr_err!("Trashed word at 0xd0 in block {} ignored in checksum calculation\n", blk);
            break;
        }
        pr_err!("Dev {}: RDB in block {} has bad checksum\n", (*(*state).disk).disk_name, blk);
        put_dev_sector(sect);
        blk = blk.wrapping_add(1);
    }

    blksize = be32_to_cpu((*rdb).rdb_BlockBytes) / 512;
    seq_buf_printf(&mut (*state).pp_buf, " RDSK ({})", blksize * 512);
    blk = be32_to_cpu((*rdb).rdb_PartitionList) as sector_t;
    put_dev_sector(sect);
    part = 1;
    while blk as i32 > 0 && part <= 16 {
        if check_mul_overflow(blk, blksize as sector_t, &mut blk) {
            pr_err!("Dev {}: overflow calculating partition block {}! Skipping partitions {} and beyond\n", (*(*state).disk).disk_name, blk, part);
            break;
        }
        data = read_part_sector(state, blk, &mut sect);
        if data.is_null() {
            pr_err!("Dev {}: unable to read partition block {}\n", (*(*state).disk).disk_name, blk);
            res = -1;
            break;
        }
        pb = data as *mut PartitionBlock;
        blk = be32_to_cpu((*pb).pb_Next) as sector_t;
        if (*pb).pb_ID != cpu_to_be32(IDNAME_PARTITION) {
            put_dev_sector(sect);
            part += 1;
            continue;
        }
        if checksum_block(pb as *const __be32,
                          (be32_to_cpu((*pb).pb_SummedLongs) & 0x7f) as i32) != 0 {
            put_dev_sector(sect);
            part += 1;
            continue;
        }

        nr_hd = be32_to_cpu((*pb).pb_Environment[NR_HD]);
        nr_sect = be32_to_cpu((*pb).pb_Environment[NR_SECT]);
        if check_mul_overflow(nr_hd, nr_sect, &mut cylblk) {
            pr_err!("Dev {}: heads*sects {} overflows u32, skipping partition!\n", (*(*state).disk).disk_name, cylblk);
            put_dev_sector(sect);
            part += 1;
            continue;
        }
        if cylblk > be32_to_cpu((*rdb).rdb_CylBlocks) {
            pr_warn!("Dev {}: cylblk {} > rdb_CylBlocks {}!\n", (*(*state).disk).disk_name, cylblk, be32_to_cpu((*rdb).rdb_CylBlocks));
        }
        if check_mul_overflow(cylblk, blksize, &mut cylblk) {
            pr_err!("Dev {}: partition {} bytes per cyl. overflows u32, skipping partition!\n", (*(*state).disk).disk_name, part);
            put_dev_sector(sect);
            part += 1;
            continue;
        }
        lo_cyl = be32_to_cpu((*pb).pb_Environment[LO_CYL]);
        start_sect = (lo_cyl as u64) * (cylblk as u64);
        hi_cyl = be32_to_cpu((*pb).pb_Environment[HI_CYL]);
        nr_sects = ((hi_cyl as u64).wrapping_sub(lo_cyl as u64).wrapping_add(1)) * (cylblk as u64);
        if nr_sects == 0 {
            put_dev_sector(sect);
            part += 1;
            continue;
        }
        if start_sect.wrapping_add(nr_sects) > u32::MAX as u64 {
            pr_warn!("Dev {}: partition {} ({}-{}) needs 64 bit device support!\n", (*(*state).disk).disk_name, part, start_sect, start_sect + nr_sects);
        }
        if check_add_overflow(start_sect as sector_t, nr_sects as sector_t, &mut end_sect) {
            pr_err!("Dev {}: partition {} ({}-{}) needs LBD device support, skipping partition!\n", (*(*state).disk).disk_name, part, start_sect, end_sect);
            put_dev_sector(sect);
            part += 1;
            continue;
        }
        put_partition(state, slot, start_sect as sector_t, nr_sects as sector_t);
        slot += 1;
        let mut dostype = [0i8; 4];
        core::ptr::copy_nonoverlapping(((*pb).pb_Environment.as_ptr().add(16)) as *const u8, dostype.as_mut_ptr() as *mut u8, 4);
        if dostype[3] < b' ' as i8 {
            seq_buf_printf(&mut (*state).pp_buf, " ({}{}{}^{})", dostype[0] as u8 as char, dostype[1] as u8 as char, dostype[2] as u8 as char, (dostype[3] + b'@' as i8) as u8 as char);
        } else {
            seq_buf_printf(&mut (*state).pp_buf, " ({}{}{}{})", dostype[0] as u8 as char, dostype[1] as u8 as char, dostype[2] as u8 as char, dostype[3] as u8 as char);
        }
        seq_buf_printf(&mut (*state).pp_buf, "(res {} spb {})", be32_to_cpu((*pb).pb_Environment[6]), be32_to_cpu((*pb).pb_Environment[4]));
        res = 1;
        put_dev_sector(sect);
        part += 1;
    }
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
