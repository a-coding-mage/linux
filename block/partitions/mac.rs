// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/mac.c
 *
 *  Code extracted from drivers/block/genhd.c
 *  Copyright (C) 1991-1998  Linus Torvalds
 *  Re-organised Feb 1998 Russell King
 */

// C dependencies: <linux/ctype.h>, "check.h", and "mac.h".

// Code to understand MacOS partition tables.

#[cfg(CONFIG_PPC_PMAC)]
#[inline]
unsafe fn mac_fix_string(stg: *mut i8, len: i32) {
    let mut i = len - 1;
    while i >= 0 && *stg.offset(i as isize) == b' ' as i8 {
        *stg.offset(i as isize) = 0;
        i -= 1;
    }
}

pub unsafe fn mac_partition(state: *mut parsed_partitions) -> i32 {
    let mut sect: Sector = core::mem::zeroed();
    let mut data: *mut u8;
    let mut slot: i32;
    let mut blocks_in_map: i32;
    let secsize: u32;
    let datasize: u32;
    let partoffset: u32;
    #[cfg(CONFIG_PPC_PMAC)]
    let mut found_root: i32 = 0;
    #[cfg(CONFIG_PPC_PMAC)]
    let mut found_root_goodness: i32 = 0;
    let mut part: *mut mac_partition;
    let mut md: *mut mac_driver_desc;

    /* Get 0th block and look at the first partition map entry. */
    md = read_part_sector(state, 0, &mut sect);
    if md.is_null() {
        return -1;
    }
    if be16_to_cpu((*md).signature) != MAC_DRIVER_MAGIC {
        put_dev_sector(sect);
        return 0;
    }
    secsize = be16_to_cpu((*md).block_size) as u32;
    put_dev_sector(sect);

    /*
     * If the "block size" is not a power of 2, things get weird - we might
     * end up with a partition straddling a sector boundary, so we wouldn't
     * be able to read a partition entry with read_part_sector().
     * Real block sizes are probably (?) powers of two, so just require
     * that.
     */
    if !is_power_of_2(secsize) {
        return -1;
    }
    datasize = round_down(secsize, 512);
    data = read_part_sector(state, datasize / 512, &mut sect) as *mut u8;
    if data.is_null() {
        return -1;
    }
    partoffset = secsize % 512;
    if partoffset + core::mem::size_of::<mac_partition>() as u32 > datasize {
        put_dev_sector(sect);
        return -1;
    }
    part = data.add(partoffset as usize) as *mut mac_partition;
    if be16_to_cpu((*part).signature) != MAC_PARTITION_MAGIC {
        put_dev_sector(sect);
        return 0; // not a MacOS disk
    }
    blocks_in_map = be32_to_cpu((*part).map_count) as i32;
    if blocks_in_map < 0 || blocks_in_map >= DISK_MAX_PARTS {
        put_dev_sector(sect);
        return 0;
    }

    if blocks_in_map >= (*state).limit {
        blocks_in_map = (*state).limit - 1;
    }

    seq_buf_puts(&mut (*state).pp_buf, " [mac]");
    slot = 1;
    while slot <= blocks_in_map {
        let pos = slot as u32 * secsize;
        put_dev_sector(sect);
        data = read_part_sector(state, pos / 512, &mut sect) as *mut u8;
        if data.is_null() {
            return -1;
        }
        part = data.add((pos % 512) as usize) as *mut mac_partition;
        if be16_to_cpu((*part).signature) != MAC_PARTITION_MAGIC {
            break;
        }
        put_partition(
            state,
            slot,
            be32_to_cpu((*part).start_block) * (secsize / 512),
            be32_to_cpu((*part).block_count) * (secsize / 512),
        );

        if strncasecmp((*part).type.as_ptr(), b"Linux_RAID\0".as_ptr(), 10) == 0 {
            (*state).parts[slot as usize].flags = ADDPART_FLAG_RAID;
        }
        #[cfg(CONFIG_PPC_PMAC)]
        {
            /* If this is the first bootable partition, tell the setup code. */
            if machine_is(powermac) {
                let mut goodness = 0;
                mac_fix_string((*part).processor.as_mut_ptr(), 16);
                mac_fix_string((*part).name.as_mut_ptr(), 32);
                mac_fix_string((*part).type.as_mut_ptr(), 32);
                if (be32_to_cpu((*part).status) & MAC_STATUS_BOOTABLE) != 0
                    && strcasecmp((*part).processor.as_ptr(), b"powerpc\0".as_ptr()) == 0
                {
                    goodness += 1;
                }
                if strcasecmp((*part).type.as_ptr(), b"Apple_UNIX_SVR2\0".as_ptr()) == 0
                    || (strncasecmp((*part).type.as_ptr(), b"Linux\0".as_ptr(), 5) == 0
                        && strcasecmp((*part).type.as_ptr(), b"Linux_swap\0".as_ptr()) != 0)
                {
                    goodness += 1;
                    let l = strnlen((*part).name.as_ptr(), core::mem::size_of_val(&(*part).name));
                    if strncmp((*part).name.as_ptr(), b"/\0".as_ptr(), core::mem::size_of_val(&(*part).name)) == 0 {
                        goodness += 1;
                    }
                    let mut i = 0;
                    while i <= l as i32 - 4 {
                        if strncasecmp((*part).name.as_ptr().add(i as usize), b"root\0".as_ptr(), 4) == 0 {
                            goodness += 2;
                            break;
                        }
                        i += 1;
                    }
                    if strncasecmp((*part).name.as_ptr(), b"swap\0".as_ptr(), 4) == 0 {
                        goodness -= 1;
                    }
                }
                if goodness > found_root_goodness {
                    found_root = slot;
                    found_root_goodness = goodness;
                }
            }
        }
        slot += 1;
    }
    #[cfg(CONFIG_PPC_PMAC)]
    if found_root_goodness != 0 {
        note_bootable_part((*(*state).disk).part0.bd_dev, found_root, found_root_goodness);
    }
    put_dev_sector(sect);
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
