// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/atari.c
 *
 *  Code extracted from drivers/block/genhd.c
 *
 *  Copyright (C) 1991-1998  Linus Torvalds
 *  Re-organised Feb 1998 Russell King
 */

// C dependencies: <linux/ctype.h>, "check.h", and "atari.h".

/* ++guenther: this should be settable by the user ("make config")?. */
// #define ICD_PARTS

#[inline]
unsafe fn valid_partition(pi: *const partition_info, hdsiz: u32) -> bool {
    (*pi).flg & 1 != 0
        && isalnum((*pi).id[0] as i32) != 0
        && isalnum((*pi).id[1] as i32) != 0
        && isalnum((*pi).id[2] as i32) != 0
        && be32_to_cpu((*pi).st) <= hdsiz
        && be32_to_cpu((*pi).st).wrapping_add(be32_to_cpu((*pi).siz)) <= hdsiz
}

#[inline]
unsafe fn ok_id(s: *const i8) -> bool {
    memcmp(s as *const core::ffi::c_void, b"GEM".as_ptr() as *const core::ffi::c_void, 3) == 0
        || memcmp(s as *const core::ffi::c_void, b"BGM".as_ptr() as *const core::ffi::c_void, 3) == 0
        || memcmp(s as *const core::ffi::c_void, b"LNX".as_ptr() as *const core::ffi::c_void, 3) == 0
        || memcmp(s as *const core::ffi::c_void, b"SWP".as_ptr() as *const core::ffi::c_void, 3) == 0
        || memcmp(s as *const core::ffi::c_void, b"RAW".as_ptr() as *const core::ffi::c_void, 3) == 0
}

pub unsafe fn atari_partition(state: *mut parsed_partitions) -> i32 {
    let mut sect: Sector = core::mem::zeroed();
    let mut rs: *mut rootsector;
    let mut pi: *mut partition_info;
    let mut extensect: u32;
    let hd_size: u32;
    let mut slot: i32;
    let mut part_fmt: i32 = 0; // 0:unknown, 1:AHDI, 2:ICD/Supra

    /* ATARI partition scheme supports 512 lba only. */
    if queue_logical_block_size((*(*state).disk).queue) != 512 {
        return 0;
    }

    rs = read_part_sector(state, 0, &mut sect);
    if rs.is_null() {
        return -1;
    }

    /* Verify this is an Atari rootsector: */
    hd_size = get_capacity((*state).disk);
    if !valid_partition(&(*rs).part[0], hd_size)
        && !valid_partition(&(*rs).part[1], hd_size)
        && !valid_partition(&(*rs).part[2], hd_size)
        && !valid_partition(&(*rs).part[3], hd_size)
    {
        /* There is no reliable magic or the like. */
        put_dev_sector(sect);
        return 0;
    }

    pi = (*rs).part.as_mut_ptr();
    seq_buf_puts(&mut (*state).pp_buf, " AHDI");
    while pi < (*rs).part.as_mut_ptr().add(4) && slot < (*state).limit {
        let mut xrs: *mut rootsector;
        let mut sect2: Sector = core::mem::zeroed();
        let mut partsect: ulong;

        if (*pi).flg & 1 == 0 {
            slot += 1;
            pi = pi.add(1);
            continue;
        }
        /* active partition */
        if memcmp((*pi).id.as_ptr() as *const core::ffi::c_void, b"XGM".as_ptr() as *const core::ffi::c_void, 3) != 0 {
            put_partition(state, slot, be32_to_cpu((*pi).st), be32_to_cpu((*pi).siz));
            slot += 1;
            pi = pi.add(1);
            continue;
        }
        /* extension partition */
        part_fmt = 1;
        seq_buf_puts(&mut (*state).pp_buf, " XGM<");
        partsect = be32_to_cpu((*pi).st) as ulong;
        extensect = partsect as u32;
        loop {
            xrs = read_part_sector(state, partsect, &mut sect2);
            if xrs.is_null() {
                printk(" block %ld read failed\n", partsect);
                put_dev_sector(sect);
                return -1;
            }
            if (*xrs).part[0].flg & 1 == 0 {
                printk("\nFirst sub-partition in extended partition is not valid!\n");
                put_dev_sector(sect2);
                break;
            }
            put_partition(state, slot, partsect + be32_to_cpu((*xrs).part[0].st) as ulong, be32_to_cpu((*xrs).part[0].siz));
            if (*xrs).part[1].flg & 1 == 0 {
                put_dev_sector(sect2);
                break;
            }
            if memcmp((*xrs).part[1].id.as_ptr() as *const core::ffi::c_void, b"XGM".as_ptr() as *const core::ffi::c_void, 3) != 0 {
                printk("\nID of extended partition is not XGM!\n");
                put_dev_sector(sect2);
                break;
            }
            partsect = be32_to_cpu((*xrs).part[1].st) as ulong + extensect as ulong;
            put_dev_sector(sect2);
            slot += 1;
            if slot == (*state).limit {
                printk("\nMaximum number of partitions reached!\n");
                break;
            }
        }
        seq_buf_puts(&mut (*state).pp_buf, " >");
        slot += 1;
        pi = pi.add(1);
    }

    if part_fmt != 1 {
        pi = (*rs).icdpart.as_mut_ptr();
        if ok_id((*pi).id.as_ptr()) {
            seq_buf_puts(&mut (*state).pp_buf, " ICD<");
            while pi < (*rs).icdpart.as_mut_ptr().add(8) && slot < (*state).limit {
                if (*pi).flg & 1 != 0 && ok_id((*pi).id.as_ptr()) {
                    put_partition(state, slot, be32_to_cpu((*pi).st), be32_to_cpu((*pi).siz));
                }
                slot += 1;
                pi = pi.add(1);
            }
            seq_buf_puts(&mut (*state).pp_buf, " >");
        }
    }
    put_dev_sector(sect);
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    1
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
